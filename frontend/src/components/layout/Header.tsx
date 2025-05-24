import { AppShell, Group, Title, Text, Box } from '@mantine/core';

type HeaderProps = {
  title: string;
  description?: string;
};

export const Header = ({ title, description }: HeaderProps) => (
  <AppShell.Header p="md" style={{ height: 80 }}>
    <Group justify="space-between" style={{ height: '100%' }}>
      <Box>
        <Title order={2}>{title}</Title>
        {description && (
          <Text size="sm" color="dimmed">
            {description}
          </Text>
        )}
      </Box>
      <Group>{/* Add navigation items here */}</Group>
    </Group>
  </AppShell.Header>
);
