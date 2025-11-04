// Trade Routes Page - Arbitrage calculator (existing functionality)

const { Layout, PageContainer, Filters, Results } = window.Components;
const { useState } = React;

function TradeRoutesPage() {
  const [offers, setOffers] = useState([]);
  const [lastFilters, setLastFilters] = useState(null);

  async function handleSearch(filters) {
    setLastFilters(filters);
    const data = await window.API.searchTradeOffers(filters);
    setOffers(data);
    return data;
  }

  return (
    <Layout>
      <PageContainer
        title="Trade Routes"
        subtitle="Find profitable arbitrage opportunities"
      >
        <Filters onSearch={handleSearch} />
        <Results data={offers} filters={lastFilters} />
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, TradeRoutesPage };
