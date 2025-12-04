import axios from 'axios';
import { serviceGenerator } from './';

const timeout = 30000;
const deployServiceIns = axios.create({
  timeout,
  baseURL: process.env.NEXT_PUBLIC_SERVER_URL,
});

export const deployApiService = serviceGenerator(deployServiceIns);
