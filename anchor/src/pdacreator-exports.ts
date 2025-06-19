// Here we export some useful types and functions for interacting with the Anchor program.
import { address } from 'gill'
import { SolanaClusterId } from '@wallet-ui/react'
import { PDACREATOR_PROGRAM_ADDRESS } from './client/js'
import PdacreatorIDL from '../target/idl/pdacreator.json'

// Re-export the generated IDL and type
export { PdacreatorIDL }

// This is a helper function to get the program ID for the Pdacreator program depending on the cluster.
export function getPdacreatorProgramId(cluster: SolanaClusterId) {
  switch (cluster) {
    case 'solana:devnet':
    case 'solana:testnet':
      // This is the program ID for the Pdacreator program on devnet and testnet.
      return address('6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF')
    case 'solana:mainnet':
    default:
      return PDACREATOR_PROGRAM_ADDRESS
  }
}

export * from './client/js'
