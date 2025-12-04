import { AxiosInstance, InternalAxiosRequestConfig } from 'axios';

interface IAxiosTypes<T> {
  data: T;
  status: number;
  statusText: string;
}

interface IResponseTypes<T> {
  status: number;
  message: string;
  data: T;
}
type TRequestHandler = <T>(
  axiosInstance: AxiosInstance,
  method: 'get' | 'post' | 'put' | 'delete' | 'postForm' | 'putForm',
  url: string,
  params?: object,
  config?: InternalAxiosRequestConfig
) => Promise<T>;

const requestHandler = <T>(
  axiosInstance: AxiosInstance,
  method: 'get' | 'post' | 'put' | 'delete' | 'postForm' | 'putForm',
  url: string,
  params: object | FormData = {},
  config?: InternalAxiosRequestConfig
): Promise<T> => {
  let response: Promise<IAxiosTypes<IResponseTypes<T>>>;
  switch (method) {
    case 'get':
      response = axiosInstance.get(url, {
        params: { ...params },
        // indexes: null is used to remove the indexes from the array params
        paramsSerializer: {
          indexes: null,
        },
        ...config,
      });
      break;
    case 'post':
      response = axiosInstance.post(url, { ...params }, { ...config });
      break;
    case 'postForm':
      response = axiosInstance.postForm(url, params, { ...config });
      break;
    case 'put':
      response = axiosInstance.put(url, { ...params }, { ...config });
      break;
    case 'putForm':
      response = axiosInstance.put(url, params, { ...config });
      break;
    case 'delete':
      response = axiosInstance.delete(url, { params: { ...params }, ...config });
      break;
  }

  return new Promise<T>((resolve, reject) => {
    response
      .then((res) => {
        const { data } = res;

        // TODO Confirm with backend
        resolve(data as any);
        // if (data.status !== 200) {
        //   reject(data);
        // } else {
        //   resolve(data.data);
        // }
      })
      .catch((error) => {
        reject(error);
      });
  });
};

const apiServicesHandle = (axiosInstance: AxiosInstance, requestHandler: TRequestHandler) => ({
  get: <T>(url: string, params?: object, config?: InternalAxiosRequestConfig) =>
    requestHandler<T>(axiosInstance, 'get', url, params, config),
  post: <T>(url: string, params?: object, config?: InternalAxiosRequestConfig) =>
    requestHandler<T>(axiosInstance, 'post', url, params, config),
  postForm: <T>(url: string, params?: object, config?: InternalAxiosRequestConfig) =>
    requestHandler<T>(axiosInstance, 'postForm', url, params, config),
  put: <T>(url: string, params?: object, config?: InternalAxiosRequestConfig) =>
    requestHandler<T>(axiosInstance, 'put', url, params, config),
  putForm: <T>(url: string, params?: object, config?: InternalAxiosRequestConfig) =>
    requestHandler<T>(axiosInstance, 'putForm', url, params, config),
  delete: <T>(url: string, params?: object, config?: InternalAxiosRequestConfig) =>
    requestHandler<T>(axiosInstance, 'delete', url, params, config),
});

const serviceGenerator = (axiosInstance: AxiosInstance) => {
  axiosInstance.interceptors.request.use(
    async (config: InternalAxiosRequestConfig) => {

      return config;
    },
    (error) => {
      return Promise.reject(error);
    }
  );

  axiosInstance.interceptors.response.use(
    (response) => {
      return response.data;
    },
    async (error) => {

      return Promise.reject(error);
    }
  );

  return apiServicesHandle(axiosInstance, requestHandler);
};

export { serviceGenerator };
