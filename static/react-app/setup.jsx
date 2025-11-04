// Setup component (<= 200 lines)

const { useEffect, useState } = React;

function Setup({ onInitialized }) {
  const [gamePath, setGamePath] = useState('');
  const [savesDir, setSavesDir] = useState('');
  const [saveFiles, setSaveFiles] = useState([]);
  const [selectedSave, setSelectedSave] = useState('');
  const [status, setStatus] = useState('');
  const [busy, setBusy] = useState(false);
  const [loadingSaves, setLoadingSaves] = useState(false);
  const [validGame, setValidGame] = useState(true);
  const [validSavesDir, setValidSavesDir] = useState(true);

  useEffect(() => {
    (async () => {
      try {
        const last = await window.API.lastPaths();
        const lg = localStorage.getItem('x4_game_path');
        const ls = localStorage.getItem('x4_saves_dir');
        const gp = lg || last.game_path || '';
        const sd = ls || last.saves_dir || '';
        if (gp) setGamePath(gp);
        if (sd) {
          setSavesDir(sd);
          await loadSaveFiles(sd);
        }
        validate(gp, sd);
      } catch {}
    })();
  }, []);

  async function loadSaveFiles(dir) {
    if (!dir) return;
    setLoadingSaves(true);
    try {
      const files = await window.API.listSaves(dir);
      setSaveFiles(files);
      if (files.length > 0) setSelectedSave(files[0].filename);
    } catch (e) {
      setSaveFiles([]);
    } finally {
      setLoadingSaves(false);
    }
  }

  function validate(gp, sd) {
    const winPathRe = /^(?:[a-zA-Z]:\\|\\\\).+/; // Drive or UNC
    const okGame = gp ? winPathRe.test(gp) : false;
    const okSavesDir = sd ? winPathRe.test(sd) : false;
    setValidGame(okGame); setValidSavesDir(okSavesDir);
    return okGame && okSavesDir;
  }

  async function handleInit() {
    if (!validate(gamePath, savesDir)) { setStatus('Please provide valid paths'); return; }
    if (!selectedSave) { setStatus('Please select a save file'); return; }
    setBusy(true); setStatus('Initializing...');
    try {
      await window.API.initSystem(gamePath, savesDir, selectedSave);
      setStatus('Initialized successfully');
      localStorage.setItem('x4_game_path', gamePath);
      localStorage.setItem('x4_saves_dir', savesDir);
      onInitialized();
    } catch (e) {
      setStatus(`Error: ${e.message || e}`);
    } finally { setBusy(false); }
  }

  async function handleSavesDirChange(newDir) {
    setSavesDir(newDir);
    validate(gamePath, newDir);
    if (newDir) {
      await loadSaveFiles(newDir);
    } else {
      setSaveFiles([]);
      setSelectedSave('');
    }
  }

  return (
    <section className="card">
      <h2>Setup</h2>
      <p className="subtle">Enter your local X4 game folder and saves directory.</p>
      <div className="grid-2">
        <div className="form-group">
          <label>Game Folder Path</label>
          <div className="input-group">
            <input className={`input-lg ${!validGame ? 'invalid' : ''}`} list="game-suggestions" value={gamePath} onChange={e=>{ setGamePath(e.target.value); validate(e.target.value, savesDir); }} placeholder="C:\\SteamLibrary\\steamapps\\common\\X4 Foundations" />
            <button className="btn-secondary" onClick={async()=>{ try{ const t=await navigator.clipboard.readText(); setGamePath(t); validate(t, savesDir);}catch{}}}>Paste</button>
            <button className="btn-secondary" onClick={()=>{ setGamePath(''); validate('', savesDir); }}>Clear</button>
          </div>
          {!validGame && <div className="subtle">Enter valid game folder path (e.g., C:\\SteamLibrary\\steamapps\\common\\X4 Foundations)</div>}
        </div>
        <div className="form-group">
          <label>Saves Directory</label>
          <div className="input-group">
            <input className={`input-lg ${!validSavesDir ? 'invalid' : ''}`} list="saves-suggestions" value={savesDir} onChange={e=>{ handleSavesDirChange(e.target.value); }} placeholder="C:\\Users\\...\\Documents\\Egosoft\\X4\\profile_name" />
            <button className="btn-secondary" onClick={async()=>{ try{ const t=await navigator.clipboard.readText(); handleSavesDirChange(t);}catch{}}}>Paste</button>
            <button className="btn-secondary" onClick={()=>{ handleSavesDirChange(''); }}>Clear</button>
          </div>
          {!validSavesDir && <div className="subtle">Enter valid saves directory path</div>}
        </div>
      </div>
      {saveFiles.length > 0 && (
        <div className="form-group" style={{marginTop: '1rem'}}>
          <label>Select Save File ({saveFiles.length} found)</label>
          <select className="input-lg" value={selectedSave} onChange={e=>setSelectedSave(e.target.value)}>
            {saveFiles.map(sf => (
              <option key={sf.filename} value={sf.filename}>
                {sf.filename} {sf.modified ? `(${sf.modified})` : ''}
              </option>
            ))}
          </select>
        </div>
      )}
      {loadingSaves && <div className="subtle">Loading save files...</div>}
      <datalist id="game-suggestions">
        <option value="C:\\SteamLibrary\\steamapps\\common\\X4 Foundations" />
        <option value="C:\\Program Files (x86)\\Steam\\steamapps\\common\\X4 Foundations" />
      </datalist>
      <datalist id="saves-suggestions">
        <option value="C:\\Users\\%USERNAME%\\Documents\\Egosoft\\X4\\profile_name" />
      </datalist>
      <div className="actions">
        <button className="btn-primary" onClick={handleInit} disabled={busy || !validGame || !validSavesDir || !selectedSave}>{busy ? 'Initializing…' : 'Initialize'}</button>
        <span className="status-text">{status}</span>
      </div>
    </section>
  );
}

window.Components = window.Components || {};
window.Components.Setup = Setup;
