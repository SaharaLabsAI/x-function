import { IDeployDetailsResDto } from '@/types/app';
import { deployApiService } from '../deployService';

export const getServiceStatus = async (id: string) => {
  return deployApiService.get<IDeployDetailsResDto>(`/apis/x402/v1/services/${id}`);
};