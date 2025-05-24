import { Card, Text, Title, Badge, LoadingOverlay, Box } from '@mantine/core';
import { useQuery } from '@tanstack/react-query';
import { healthCheck } from '../../api/health';

export const HealthCheck = () => {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ['health'],
    queryFn: healthCheck,
    retry: 1,
    refetchOnWindowFocus: false,
  });

  const status = data?.status;
  const version = data?.version;

  const getStatusBadge = () => {
    if (isLoading) return <Badge color="blue">Checking...</Badge>;
    if (isError) return <Badge color="red">Error</Badge>;
    if (status === 'ok') return <Badge color="green">Healthy</Badge>;
    return <Badge color="yellow">Unknown</Badge>;
  };

  return (
    <Card shadow="sm" p="lg" radius="md" withBorder>
      <Box>
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginBottom: '1rem',
          }}
        >
          <Title order={3}>API Health Status</Title>
          {getStatusBadge()}
        </div>

        {isLoading && <LoadingOverlay visible={true} />}

        {isError ? (
          <Text color="red">Error connecting to the API: {error?.message || 'Unknown error'}</Text>
        ) : (
          <div>
            <Text>Status: {status || 'N/A'}</Text>
            <Text size="sm" color="dimmed">
              Version: {version || 'N/A'}
            </Text>
          </div>
        )}
      </Box>
    </Card>
  );
};

export default HealthCheck;
