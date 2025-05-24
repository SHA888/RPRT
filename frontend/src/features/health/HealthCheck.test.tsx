import { render, screen } from '@testing-library/react';
import { QueryClient } from '@tanstack/react-query';
import { QueryClientProvider } from '@tanstack/react-query';
import { MantineProvider } from '@mantine/core';
import { describe, it, expect } from 'vitest';
import type { ReactNode } from 'react';
import { HealthCheck } from './HealthCheck';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
    },
  },
});

const wrapper = ({ children }: { children: ReactNode }) => (
  <QueryClientProvider client={queryClient}>
    <MantineProvider>
      {children}
    </MantineProvider>
  </QueryClientProvider>
);

describe('HealthCheck', () => {
  it('renders loading state initially', () => {
    render(<HealthCheck />, { wrapper });
    expect(screen.getByText('Checking...')).toBeInTheDocument();
  });
});
