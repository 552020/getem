import { ApiResponse } from '@calimero-network/calimero-client';

import {
  ApprovalsCount,
  ContextDetails,
  ContextVariables,
  ContractApi,
  ContractProposal,
  Members,
} from '../contractApi';
import { getNodeUrlFromLocalStorage } from '../../utils/storage';
import axios from 'axios';
import { getConfigAndJwt } from './LogicApiDataSource';

export interface GetProposalsRequest {
  offset: number;
  limit: number;
}

export class ContextApiDataSource implements ContractApi {
  async getContractProposals(
    request: GetProposalsRequest,
  ): ApiResponse<ContractProposal[]> {
    try {
      const { jwtObject, error } = getConfigAndJwt();
      if (error) {
        return { error };
      }

      const apiEndpoint = `${getNodeUrlFromLocalStorage()}/admin-api/contexts/${jwtObject.context_id}/proposals`;
      const body = request;

      const response = await axios.post(apiEndpoint, body, {
        headers: {
          'Content-Type': 'application/json',
        },
      });

      return {
        data: response.data ?? [],
        error: null,
      };
    } catch (error) {
      return {
        data: null,
        error: error as Error,
      };
    }
  }

  async getProposalApprovals(proposalId: string): ApiResponse<ApprovalsCount> {
    try {
      const { jwtObject, error } = getConfigAndJwt();
      if (error) {
        return { error };
      }
      const apiEndpoint = `${getNodeUrlFromLocalStorage()}/admin-api/contexts/${jwtObject.context_id}/proposals/${proposalId}/approvals/users`;

      const response = await axios.get(apiEndpoint);

      return {
        data: response.data ?? [],
        error: null,
      };
    } catch (error) {
      return {
        data: null,
        error: error as Error,
      };
    }
  }

  async getNumOfProposals(): ApiResponse<number> {
    try {
      const { jwtObject, error } = getConfigAndJwt();
      if (error) {
        return { error };
      }

      const apiEndpointLimit = `${getNodeUrlFromLocalStorage()}/admin-api/contexts/${jwtObject.context_id}/proposals/count`;
      const limitResponse = await axios.get(apiEndpointLimit);

      const apiEndpoint = `${getNodeUrlFromLocalStorage()}/admin-api/contexts/${jwtObject.context_id}/proposals`;
      const body = {
        offset: 0,
        limit: limitResponse.data.data,
      };

      const response = await axios.post(apiEndpoint, body, {
        headers: {
          'Content-Type': 'application/json',
        },
      });

      return {
        data: response.data.data.length ?? 0,
        error: null,
      };
    } catch (error) {
      return {
        data: null,
        error: error as Error,
      };
    }
  }

  async getContextVariables(): ApiResponse<ContextVariables[]> {
    try {
      const { jwtObject, error } = getConfigAndJwt();
      if (error) {
        return { error };
      }

      const apiEndpoint = `${getNodeUrlFromLocalStorage()}/admin-api/contexts/${jwtObject.context_id}/proposals/context-storage-entries`;
      const body = {
        offset: 0,
        limit: 10,
      };

      const response = await axios.post(apiEndpoint, body, {
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.data.data) {
        return {
          data: [],
          error: null,
        };
      }

      // Convert both key and value from Vec<u8> to string
      const parsedData = response.data.data.map((item: any) => ({
        key: new TextDecoder().decode(new Uint8Array(item.key)),
        value: new TextDecoder().decode(new Uint8Array(item.value)),
      }));

      return {
        data: parsedData ?? [],
        error: null,
      };
    } catch (error) {
      return {
        data: null,
        error: error as Error,
      };
    }
  }

  async getContextMembers(): ApiResponse<Members[]> {
    // TODO: Backend API endpoint not implemented yet
    // Mock implementation returning empty array
    console.warn('getContextMembers: Backend API endpoint not implemented yet');
    return {
      data: [],
      error: null,
    };
  }

  async getContextMembersCount(): ApiResponse<number> {
    // TODO: Backend API endpoint not implemented yet
    // Mock implementation returning 0
    console.warn(
      'getContextMembersCount: Backend API endpoint not implemented yet',
    );
    return {
      data: 0,
      error: null,
    };
  }

  async getContextDetails(contextId: string): ApiResponse<ContextDetails> {
    // TODO: Backend API endpoint not implemented yet
    // Mock implementation returning empty object
    // console.log to shut up the warning
    console.log('contextId', contextId);
    console.warn('getContextDetails: Backend API endpoint not implemented yet');
    return {
      data: {} as ContextDetails,
      error: null,
    };
  }

  async deleteProposal(proposalId: string): ApiResponse<void> {
    try {
      const { jwtObject, error } = getConfigAndJwt();
      if (error) {
        return { error };
      }

      const apiEndpoint = `${getNodeUrlFromLocalStorage()}/admin-api/contexts/${jwtObject.context_id}/proposals/${proposalId}`;
      await axios.delete(apiEndpoint);

      return {
        data: undefined,
        error: null,
      };
    } catch (error) {
      return {
        data: null,
        error: error as Error,
      };
    }
  }
}
