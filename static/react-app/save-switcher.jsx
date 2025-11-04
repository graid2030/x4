// SaveSwitcher component (<= 200 lines)

const { useEffect, useState } = React;

function SaveSwitcher({ onSaveChanged }) {
  const [savesDir, setSavesDir] = useState('');
  const [gamePath, setGamePath] = useState('');
  const [saveFiles, setSaveFiles] = useState([]);
  const [selectedSave, setSelectedSave] = useState('');
  const [loading, setLoading] = useState(false);
  const [expanded, setExpanded] = useState(false);

  useEffect(() => {
    loadSavedPaths();
  }, []);

  async function loadSavedPaths() {
    try {
      const last = await window.API.lastPaths();
      const gp = localStorage.getItem('x4_game_path') || last.game_path || '';
      const sd = localStorage.getItem('x4_saves_dir') || last.saves_dir || '';

      setGamePath(gp);
      setSavesDir(sd);

      if (sd) {
        await loadSaveFiles(sd);
      }
    } catch (e) {
      console.error('Failed to load paths:', e);
    }
  }

  async function loadSaveFiles(dir) {
    if (!dir) return;
    setLoading(true);
    try {
      const files = await window.API.listSaves(dir);
      setSaveFiles(files);
      if (files.length > 0 && !selectedSave) {
        setSelectedSave(files[0].filename);
      }
    } catch (e) {
      console.error('Failed to load saves:', e);
      setSaveFiles([]);
    } finally {
      setLoading(false);
    }
  }

  async function handleSaveChange(newSave) {
    if (!newSave || !gamePath || !savesDir) return;

    setSelectedSave(newSave);
    setLoading(true);

    try {
      await window.API.initSystem(gamePath, savesDir, newSave);
      if (onSaveChanged) {
        onSaveChanged(newSave);
      }
    } catch (e) {
      alert(`Failed to switch save: ${e.message || e}`);
    } finally {
      setLoading(false);
      setExpanded(false);
    }
  }

  async function handleRefresh() {
    if (savesDir) {
      await loadSaveFiles(savesDir);
    }
  }

  if (!savesDir || saveFiles.length === 0) {
    return null;
  }

  return (
    <div className="card" style={{marginBottom: '1rem'}}>
      <div style={{display: 'flex', justifyContent: 'space-between', alignItems: 'center', gap: '1rem'}}>
        <div style={{flex: 1}}>
          <label className="subtle" style={{fontSize: '0.85em', marginBottom: '0.25rem', display: 'block'}}>
            Current Save File
          </label>
          {!expanded ? (
            <div style={{display: 'flex', alignItems: 'center', gap: '0.5rem'}}>
              <span style={{fontWeight: 600}}>{selectedSave || 'No save selected'}</span>
              <button
                className="btn-secondary"
                onClick={() => setExpanded(true)}
                style={{padding: '4px 12px', fontSize: '0.85em'}}
              >
                Change
              </button>
            </div>
          ) : (
            <div style={{display: 'flex', alignItems: 'center', gap: '0.5rem'}}>
              <select
                className="input-lg"
                value={selectedSave}
                onChange={e => handleSaveChange(e.target.value)}
                disabled={loading}
                style={{flex: 1}}
              >
                {saveFiles.map(sf => (
                  <option key={sf.filename} value={sf.filename}>
                    {sf.filename} {sf.modified ? `(${sf.modified})` : ''}
                  </option>
                ))}
              </select>
              <button
                className="btn-secondary"
                onClick={handleRefresh}
                disabled={loading}
                title="Refresh save list"
              >
                🔄
              </button>
              <button
                className="btn-secondary"
                onClick={() => setExpanded(false)}
                disabled={loading}
              >
                Cancel
              </button>
            </div>
          )}
        </div>
        <div className="subtle" style={{fontSize: '0.85em'}}>
          {saveFiles.length} save{saveFiles.length !== 1 ? 's' : ''} found
        </div>
      </div>
      {loading && (
        <div className="subtle" style={{marginTop: '0.5rem', fontSize: '0.85em'}}>
          Loading...
        </div>
      )}
    </div>
  );
}

window.Components = window.Components || {};
window.Components.SaveSwitcher = SaveSwitcher;
