import { AppShell, Container } from '@mantine/core';
import { ReactNode } from 'react';
import { Header } from './Header';

type LayoutProps = {
  children: ReactNode;
  title?: string;
  description?: string;
};

export const Layout = ({
  children,
  title = 'RPRT App',
  description = 'Rust PostgreSQL React TypeScript Template',
}: LayoutProps) => {
  return (
    <AppShell
      padding="md"
      header={<Header title={title} description={description} />}
      styles={(theme) => ({
        main: {
          backgroundColor:
            theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0],
          minHeight: '100vh',
        },
      })}
    >
      <Container size="lg">{children}</Container>
    </AppShell>
  );
};
