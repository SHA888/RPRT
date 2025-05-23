import client from './client';

export interface HealthCheckResponse {
  status: string;
  version: string;
}

export const healthCheck = async (): Promise<HealthCheckResponse> => {
  const response = await client.get<HealthCheckResponse>('/health');
  return response.data;
};

export default {
  healthCheck,
};
