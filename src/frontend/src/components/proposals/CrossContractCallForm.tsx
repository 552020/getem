import React from 'react';
import { ProposalData } from './CreateProposalPopup';

interface CrossContractCallFormProps {
  proposalForm: ProposalData;
  handleInputChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>,
  ) => void;
  handleArgumentChange: (
    index: number,
    field: 'key' | 'value',
    value: string,
  ) => void;
  removeArgument: (index: number) => void;
  addArgument: () => void;
}

export default function CrossContractCallForm({
  proposalForm,
  handleInputChange,
  handleArgumentChange,
  removeArgument,
  addArgument,
}: CrossContractCallFormProps) {
  return (
    <>
      <div className="mb-4">
        <label htmlFor="contractId" className="block mb-2">
          Contract ID
        </label>
        <input
          type="text"
          id="contractId"
          name="contractId"
          placeholder="contract address"
          value={proposalForm.contractId}
          onChange={handleInputChange}
          required
          className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
        />
      </div>
      <div className="mb-4">
        <label htmlFor="methodName" className="block mb-2">
          Method Name
        </label>
        <input
          type="text"
          id="methodName"
          name="methodName"
          placeholder="create_post"
          value={proposalForm.methodName}
          onChange={handleInputChange}
          required
          className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
        />
      </div>
      <div className="mb-4">
        <label htmlFor="deposit" className="block mb-2">
          Deposit
        </label>
        <input
          type="text"
          id="deposit"
          name="deposit"
          value={proposalForm.deposit}
          onChange={handleInputChange}
          placeholder="0"
          required
          className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
        />
      </div>
      <div className="mb-4">
        <label className="block mb-2">Arguments</label>
        <div className="max-h-[150px] overflow-y-auto">
          {proposalForm.arguments.map(
            (arg: { key: string; value: string }, index: number) => (
              <div key={index} className="flex gap-4 items-end mb-4">
                <div className="flex-1">
                  <input
                    type="text"
                    placeholder="key"
                    value={arg.key}
                    onChange={(e) =>
                      handleArgumentChange(index, 'key', e.target.value)
                    }
                    required
                    className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
                  />
                </div>
                <div className="flex-1">
                  <input
                    type="text"
                    placeholder="value"
                    value={arg.value}
                    onChange={(e) =>
                      handleArgumentChange(index, 'value', e.target.value)
                    }
                    required
                    className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
                  />
                </div>
                <button
                  type="button"
                  onClick={() => removeArgument(index)}
                  className="px-4 py-1 rounded-lg bg-[#666] text-white cursor-pointer"
                >
                  Remove
                </button>
              </div>
            ),
          )}
        </div>
        <button
          type="button"
          onClick={addArgument}
          className="px-4 py-1 rounded-lg bg-[#666] text-white cursor-pointer"
        >
          Add Argument
        </button>
      </div>
    </>
  );
}
