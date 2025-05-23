import { Layout } from './components/layout/Layout';
import { HealthCheck } from './features/health/HealthCheck';

function App() {
  return (
    <Layout title="RPRT App" description="Rust PostgreSQL React TypeScript Template">
      <HealthCheck />
    </Layout>
  );
}

export default App;
