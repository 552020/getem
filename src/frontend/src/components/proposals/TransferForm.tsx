import React from 'react';
import { ProposalData } from './CreateProposalPopup';

interface TransferFormProps {
  proposalForm: ProposalData;
  handleInputChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>,
  ) => void;
}

export default function TransferForm({
  proposalForm,
  handleInputChange,
}: TransferFormProps) {
  return (
    <>
      <div className="mb-4">
        <label htmlFor="receiverId" className="block mb-2">
          Receiver ID
        </label>
        <input
          type="text"
          id="receiverId"
          name="receiverId"
          value={proposalForm.receiverId}
          onChange={handleInputChange}
          required
          className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
        />
      </div>
      <div className="mb-4">
        <label htmlFor="amount" className="block mb-2">
          Amount
        </label>
        <input
          type="text"
          id="amount"
          name="amount"
          value={proposalForm.amount}
          onChange={handleInputChange}
          required
          className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
        />
      </div>
    </>
  );
}
