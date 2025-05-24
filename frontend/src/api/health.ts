import client from './client';

export interface HealthCheckResponse {
  status: string;
  version: string;
}

export const healthCheck = async (): Promise<HealthCheckResponse> => {
  try {
    const response = await client.get<HealthCheckResponse>('/health');
    if (!response) {
      throw new Error('No response received from server');
    }
    // The response interceptor already returns the data, so we can cast it directly
    return response as unknown as HealthCheckResponse;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`Health check failed: ${error.message}`);
    }
    throw new Error('Health check failed: Unknown error');
  }
};

export default {
  healthCheck,
};
