import { ApiResponse } from '@calimero-network/calimero-client';

export interface ContextDetails {
  id: string;
}

export interface Members {
  publicKey: string;
}

export interface ProposalAction {
  scope: string;
  params: {
    amount: number;
    receiver_id: string;
  };
}

export interface ContractProposal {
  id: string;
  author_id: string;
  actions: ProposalAction[];
}

export interface CalimeroProposalMetadata {
  timestamp: number;
}

export interface Message {
  publicKey: string;
}

export interface ApprovalsCount {
  proposal_id: string;
  num_approvals: number;
}

export interface ContextVariables {
  key: string;
  value: string;
}

export interface ContractApi {
  //Contract
  getContractProposals(request: {
    offset: number;
    limit: number;
  }): ApiResponse<ContractProposal[]>;
  getNumOfProposals(): ApiResponse<number>;
  getProposalApprovals(proposalId: string): ApiResponse<ApprovalsCount>;
  getContextDetails(contextId: string): ApiResponse<ContextDetails>;
  getContextMembers(): ApiResponse<Members[]>;
  getContextMembersCount(): ApiResponse<number>;
  deleteProposal(proposalId: string): ApiResponse<void>;
  getContextVariables(): ApiResponse<ContextVariables[]>;
}

// async removeProposal(proposalId: String): ApiResponse<boolean> {
//   return await this.client.delete<boolean>(
//     `${this.endpoint}/admin-api/contexts/${this.contextId}/proposals/${proposalId}`,
//   );
// }

// async getProposalMessage(
//   proposalId: String,
//   messageId: String,
// ): ApiResponse<Message> {
//   return await this.client.get<Message>(
//     `${this.endpoint}/admin-api/contexts/${this.contextId}/proposals/${proposalId}/messages/${messageId}`,
//   );
// }

// async getProposalMessages(proposalId: String): ApiResponse<Message[]> {
//   return await this.client.get<Message[]>(
//     `${this.endpoint}/admin-api/contexts/${this.contextId}/proposals/${proposalId}/messages`,
//   );
// }
// async approveProposal(proposalId: String): ApiResponse<boolean> {
//   return await this.client.post<boolean>(
//     `${this.endpoint}/admin-api/contexts/${this.contextId}/proposals/${proposalId}/vote`,
//   );
// }
