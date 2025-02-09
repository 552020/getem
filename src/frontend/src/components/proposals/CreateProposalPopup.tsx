import React, { useState } from 'react';
import ActionsDropdown, { ActionTypes } from './ActionsDropdown';
import CrossContractCallForm from './CrossContractCallForm';
import TransferForm from './TransferForm';
import SetContextVariableForm from './SetContextVariableForm';
import ChangeApprovalsNeededForm from './ChangeApprovalsNeededForm';
import MaxActiveProposalsForm from './MaxActiveProposalsForm';

export interface ProposalData {
  actionType: string;
  contractId: string;
  methodName: string;
  arguments: { key: string; value: string }[];
  deposit: string;
  receiverId: string;
  amount: string;
  contextVariables: { key: string; value: string }[];
  minApprovals: string;
  maxActiveProposals: string;
}

interface CreateProposalPopupProps {
  setIsModalOpen: (isModalOpen: boolean) => void;
  createProposal: (proposalForm: ProposalData) => Promise<void>;
}

export default function CreateProposalPopup({
  setIsModalOpen,
  createProposal,
}: CreateProposalPopupProps) {
  const [proposalForm, setProposalForm] = useState({
    actionType: 'Cross contract call',
    contractId: '',
    methodName: '',
    arguments: [{ key: '', value: '' }],
    deposit: '',
    receiverId: '',
    amount: '',
    contextVariables: [{ key: '', value: '' }],
    minApprovals: '',
    maxActiveProposals: '',
  });

  const handleInputChange = (
    e: React.ChangeEvent<
      HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement
    >,
  ) => {
    const { name, value } = e.target;

    setProposalForm((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleArgumentChange = (
    index: number,
    field: 'key' | 'value',
    value: string,
  ) => {
    setProposalForm((prev) => {
      const newArgs = [...prev.arguments];
      newArgs[index] = {
        ...newArgs[index],
        [field]: value,
      };
      return {
        ...prev,
        arguments: newArgs,
      };
    });
  };

  const addContextVariable = () => {
    setProposalForm((prev) => ({
      ...prev,
      contextVariables: [...prev.contextVariables, { key: '', value: '' }],
    }));
  };

  const removeContextVariable = (index: number) => {
    setProposalForm((prev) => ({
      ...prev,
      contextVariables: prev.contextVariables.filter((_, i) => i !== index),
    }));
  };

  const addArgument = () => {
    setProposalForm((prev) => ({
      ...prev,
      arguments: [...prev.arguments, { key: '', value: '' }],
    }));
  };

  const removeArgument = (index: number) => {
    setProposalForm((prev) => ({
      ...prev,
      arguments: prev.arguments.filter((_, i) => i !== index),
    }));
  };

  const handleContextVariableChange = (
    index: number,
    field: 'key' | 'value',
    value: string,
  ) => {
    setProposalForm((prev: any) => {
      const newVariables = [...prev.contextVariables];
      newVariables[index] = {
        ...newVariables[index],
        [field]: value,
      };
      return {
        ...prev,
        contextVariables: newVariables,
      };
    });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsModalOpen(false);
    await createProposal(proposalForm);

    setProposalForm({
      actionType: 'Cross contract call',
      contractId: '',
      methodName: '',
      arguments: [{ key: '', value: '' }],
      deposit: '',
      receiverId: '',
      amount: '',
      contextVariables: [{ key: '', value: '' }],
      minApprovals: '',
      maxActiveProposals: '',
    });
  };

  return (
    <div
      className="fixed inset-0 bg-black/70 flex justify-center items-center z-[1000]"
      onClick={() => setIsModalOpen(false)}
    >
      <div
        className="bg-[#1e1e1e] p-4 rounded-lg w-[90%] max-w-[500px] text-white"
        onClick={(e) => e.stopPropagation()}
      >
        <h2 className="pb-2 m-0">Create New Proposal</h2>
        <form onSubmit={handleSubmit}>
          <ActionsDropdown
            actionType={proposalForm.actionType}
            handleInputChange={handleInputChange}
          />
          {proposalForm.actionType === ActionTypes.CROSS_CONTRACT_CALL && (
            <CrossContractCallForm
              proposalForm={proposalForm}
              handleInputChange={handleInputChange}
              handleArgumentChange={handleArgumentChange}
              removeArgument={removeArgument}
              addArgument={addArgument}
            />
          )}
          {proposalForm.actionType === ActionTypes.TRANSFER && (
            <TransferForm
              proposalForm={proposalForm}
              handleInputChange={handleInputChange}
            />
          )}

          {proposalForm.actionType === ActionTypes.SET_CONTEXT_VARIABLE && (
            <SetContextVariableForm
              proposalForm={proposalForm}
              handleContextVariableChange={handleContextVariableChange}
              removeContextVariable={removeContextVariable}
              addContextVariable={addContextVariable}
            />
          )}

          {proposalForm.actionType === ActionTypes.CHANGE_APPROVALS_NEEDED && (
            <ChangeApprovalsNeededForm
              proposalForm={proposalForm}
              handleInputChange={handleInputChange}
            />
          )}

          {proposalForm.actionType ===
            ActionTypes.CHANGE_MAX_ACTIVE_PROPOSALS && (
            <MaxActiveProposalsForm
              proposalForm={proposalForm}
              handleInputChange={handleInputChange}
            />
          )}

          <div className="flex justify-end gap-4 mt-4">
            <button
              type="button"
              onClick={() => setIsModalOpen(false)}
              className="text-white px-4 py-1 m-1 rounded-lg text-base bg-[#666] cursor-pointer flex justify-center border-none outline-none"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="text-white px-4 py-1 m-1 rounded-lg text-base bg-[#5dbb63] cursor-pointer flex justify-center border-none outline-none"
            >
              Create Proposal
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
