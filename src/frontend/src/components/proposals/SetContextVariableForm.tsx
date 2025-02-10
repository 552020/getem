import { ProposalData } from './CreateProposalPopup';

interface SetContextVariableFormProps {
  proposalForm: ProposalData;
  handleContextVariableChange: (
    index: number,
    field: 'key' | 'value',
    value: string,
  ) => void;
  removeContextVariable: (index: number) => void;
  addContextVariable: () => void;
}

export default function SetContextVariableForm({
  proposalForm,
  handleContextVariableChange,
  removeContextVariable,
  addContextVariable,
}: SetContextVariableFormProps) {
  return (
    <>
      <div className="max-h-[150px] overflow-y-auto">
        {proposalForm.contextVariables.map(
          (variable: { key: string; value: string }, index: number) => (
            <div key={index} className="flex gap-4 items-end mb-4">
              <div className="flex-1">
                <input
                  type="text"
                  placeholder="key"
                  value={variable.key}
                  onChange={(e) =>
                    handleContextVariableChange(index, 'key', e.target.value)
                  }
                  required
                  className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
                />
              </div>
              <div className="flex-1">
                <input
                  type="text"
                  placeholder="value"
                  value={variable.value}
                  onChange={(e) =>
                    handleContextVariableChange(index, 'value', e.target.value)
                  }
                  required
                  className="w-full p-2 rounded bg-[#333] border border-[#444] text-white"
                />
              </div>
              <button
                type="button"
                onClick={() => removeContextVariable(index)}
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
        onClick={addContextVariable}
        className="px-4 py-1 rounded-lg bg-[#666] text-white cursor-pointer"
      >
        Add Variable
      </button>
    </>
  );
}
