// 404 Not Found Page

const { Layout, PageContainer } = window.Components;
const { Link } = window.Router;

function NotFoundPage() {
  return (
    <Layout>
      <PageContainer
        title="404 - Page Not Found"
        subtitle="The page you're looking for doesn't exist"
      >
        <div className="card">
          <p>The requested page could not be found.</p>
          <p>
            <Link to="/">← Go back to Dashboard</Link>
          </p>
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, NotFoundPage };
