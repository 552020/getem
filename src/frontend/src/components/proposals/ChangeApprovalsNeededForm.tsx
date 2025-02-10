import React from 'react';
import { ProposalData } from './CreateProposalPopup';

interface ChangeApprovalsNeededFormProps {
  proposalForm: ProposalData;
  handleInputChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>,
  ) => void;
}

export default function ChangeApprovalsNeededForm({
  proposalForm,
  handleInputChange,
}: ChangeApprovalsNeededFormProps) {
  return (
    <div className="mb-4">
      <label htmlFor="minApprovals" className="block mb-2">
        Minimum Approvals Required
      </label>
      <input
        type="number"
        id="minApprovals"
        name="minApprovals"
        placeholder="2"
        value={proposalForm.minApprovals}
        onChange={handleInputChange}
        min="1"
        required
        className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
      />
    </div>
  );
}
