import { AppShell, Header as MantineHeader, Group, Title, Text, Box } from '@mantine/core';

type HeaderProps = {
  title: string;
  description?: string;
};

export const Header = ({ title, description }: HeaderProps) => (
  <MantineHeader height={80} p="md">
    <Group position="apart" sx={{ height: '100%' }}>
      <Box>
        <Title order={2}>{title}</Title>
        {description && (
          <Text size="sm" color="dimmed">
            {description}
          </Text>
        )}
      </Box>
      <Group>
        {/* Add navigation items here */}
      </Group>
    </Group>
  </MantineHeader>
);
