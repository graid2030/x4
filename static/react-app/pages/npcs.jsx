// NPCs Page - Crew management

const { Layout, PageContainer } = window.Components;

function NPCsPage() {
  return (
    <Layout>
      <PageContainer
        title="NPCs"
        subtitle="Manage your hired crew members"
      >
        <div className="card">
          <h2>NPC Crew Management</h2>
          <p>This page will show:</p>
          <ul>
            <li>Table of all player-owned NPCs</li>
            <li>Skills: Piloting, Engineering, Boarding, Management, Morale</li>
            <li>Current assignments (ship/station)</li>
            <li>Role detection (pilot, engineer, manager, trader)</li>
            <li>Filter by role, sort by skills</li>
            <li>Recommendations for unassigned NPCs</li>
          </ul>
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, NPCsPage };
