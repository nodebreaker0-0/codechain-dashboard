import { Action } from "./actions";
import { ChainNetworks, NodeInfo } from "./requests/types";
const merge = require("deepmerge").default;

export interface RootState {
  nodeInfo: {
    [socketAddr: string]: NodeInfo;
  };
  chainNetworks: ChainNetworks | undefined;
}

const initialState: RootState = {
  nodeInfo: {},
  chainNetworks: undefined
};

export const appReducer = (state = initialState, action: Action) => {
  switch (action.type) {
    case "SetChainNetworks": {
      const chainNetworks = action.data;
      return {
        ...state,
        chainNetworks
      };
    }
    case "SetNodeInfo": {
      const nodeInfo = {
        ...state.nodeInfo,
        [action.socketAddr]: action.data
      };
      return {
        ...state,
        nodeInfo
      };
    }
    case "UpdateChainNetworks": {
      const updatedChainNetworks = merge(state.chainNetworks, action);
      return {
        ...state,
        updatedChainNetworks
      };
    }
    case "UpdateNodeInfo":
      const updatedNodeInfo = {
        ...state.nodeInfo,
        [action.socketAddr]: merge(
          state.nodeInfo[action.socketAddr],
          action.data
        )
      };
      return {
        ...state,
        updatedNodeInfo
      };
  }
  return state;
};
