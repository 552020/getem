interface FunctionCallProps {
  receiver_id: string;
  method_name: string;
  args: string;
  deposit: number;
  gas: number;
}

interface ContextValueProps {
  key: number[];
  value: number[];
}

interface ApprovalsParams {
  num_approvals: number;
}

interface LimitParams {
  active_proposals_limit: number;
}

interface TransferParams {
  receiver_id: string;
  amount: number;
}

interface Action {
  scope: string;
  params:
    | LimitParams
    | TransferParams
    | ApprovalsParams
    | ContextValueProps
    | FunctionCallProps;
}
interface ActionsProps {
  actions: Action[];
}

// interface TableHeaderProps {
//   columns: number;
//   bgColor?: boolean;
// }

export default function Actions({ actions }: ActionsProps) {
  const getColumnCount = (scope: string) => {
    switch (scope) {
      case 'Transfer':
      case 'SetContextValue':
        return 3;
      case 'SetActiveProposalsLimit':
      case 'SetNumApprovals':
        return 2;
      case 'ExternalFunctionCall':
        return 5;
      default:
        return 1;
    }
  };

  const renderActionContent = (action: Action) => {
    switch (action.scope) {
      case 'Transfer':
        return (
          <>
            <div>Scope</div>
            <div>Amount</div>
            <div>Receiver ID</div>
          </>
        );
      case 'SetContextValue':
        return (
          <>
            <div>Scope</div>
            <div>Key</div>
            <div>Value ID</div>
          </>
        );
      case 'SetActiveProposalsLimit':
        return (
          <>
            <div>Scope</div>
            <div>Active Proposals Limit</div>
          </>
        );
      case 'SetNumApprovals':
        return (
          <>
            <div>Scope</div>
            <div>Number of Approvals</div>
          </>
        );
      case 'ExternalFunctionCall':
        return (
          <>
            <div>Scope</div>
            <div>Receiver ID</div>
            <div>Method</div>
            <div>Deposit</div>
            <div>Gas</div>
          </>
        );
      default:
        return <div>Scope</div>;
    }
  };

  const renderActionValues = (action: Action) => {
    switch (action.scope) {
      case 'Transfer':
        const transferParams = action.params as TransferParams;
        return (
          <>
            <div>{action.scope}</div>
            <div>{transferParams.amount}</div>
            <div>{transferParams.receiver_id}</div>
          </>
        );
      case 'SetContextValue':
        const contextValueParams = action.params as ContextValueProps;
        return (
          <>
            <div>{action.scope}</div>
            <div>{String.fromCharCode(...contextValueParams.key)}</div>
            <div>{String.fromCharCode(...contextValueParams.value)}</div>
          </>
        );
      case 'SetActiveProposalsLimit':
        const limitParams = action.params as LimitParams;
        return (
          <>
            <div>{action.scope}</div>
            <div>{limitParams.active_proposals_limit}</div>
          </>
        );
      case 'SetNumApprovals':
        const approvalParams = action.params as ApprovalsParams;
        return (
          <>
            <div>{action.scope}</div>
            <div>{approvalParams.num_approvals}</div>
          </>
        );
      case 'ExternalFunctionCall':
        const functionParams = action.params as FunctionCallProps;
        return (
          <>
            <div>{action.scope}</div>
            <div>{functionParams.receiver_id}</div>
            <div>{functionParams.method_name}</div>
            <div>{functionParams.deposit}</div>
            <div>{functionParams.gas}</div>
          </>
        );
      default:
        return <div>{action.scope}</div>;
    }
  };

  return (
    <>
      <div
        className={`grid gap-2 ${
          actions[0]
            ? `grid-cols-${getColumnCount(actions[0].scope)}`
            : 'grid-cols-1'
        } ${actions[0] ? 'bg-orange-500' : 'bg-transparent'}`}
      >
        {actions[0] && renderActionContent(actions[0])}
      </div>
      <div>
        {actions.map((action, index) => (
          <div
            key={index}
            className={`grid gap-2 ${`grid-cols-${getColumnCount(action.scope)}`}`}
          >
            {renderActionValues(action)}
          </div>
        ))}
      </div>
    </>
  );
}
