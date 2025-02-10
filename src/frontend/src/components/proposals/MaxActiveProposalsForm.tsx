import React from 'react';
import { ProposalData } from './CreateProposalPopup';

interface MaxActiveProposalsFormProps {
  proposalForm: ProposalData;
  handleInputChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>,
  ) => void;
}

export default function MaxActiveProposalsForm({
  proposalForm,
  handleInputChange,
}: MaxActiveProposalsFormProps) {
  return (
    <div className="mb-4">
      <label htmlFor="maxActiveProposals" className="block mb-2">
        Maximum Active Proposals
      </label>
      <input
        type="number"
        id="maxActiveProposals"
        name="maxActiveProposals"
        placeholder="10"
        value={proposalForm.maxActiveProposals}
        onChange={handleInputChange}
        min="1"
        required
        className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
      />
    </div>
  );
}
