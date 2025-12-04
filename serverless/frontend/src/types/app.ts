export interface ICreatedServiceInfo {
  id: string,
  name: string
}

export interface IDeployDetailsResDto {
  id: string;
  name: string;
  ready: boolean;
  url: string;
  message: string;
  extra: {
    details: {
      phase: string;
      status: string;
    }[];
  }
}