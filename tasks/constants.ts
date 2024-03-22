import * as anchor from '@coral-xyz/anchor';
import { Program, AnchorProvider } from '@coral-xyz/anchor';
import base58 from 'bs58';
import { Connection, clusterApiUrl } from '@solana/web3.js';

import { IDL, AmmV3 } from './raydiumIDL';
import { RaydiumPosition } from './position';
import { LiquidityLocker } from '../target/types/liquidity_locker';
import keys from '../keys.json';

const provider = AnchorProvider.env();
anchor.setProvider(provider);

export const CONNECTION = new Connection(clusterApiUrl('devnet'));

export const DEPLOYER = anchor.web3.Keypair.fromSecretKey(base58.decode(keys.deployer));

export const program = anchor.workspace.LiquidityLocker as Program<LiquidityLocker>;

export const raydiumProgramId = new anchor.web3.PublicKey(
  'devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH'
);
export const raydiumProgram = new anchor.Program(IDL, raydiumProgramId) as Program<AmmV3>;

export const position = new RaydiumPosition('BGWnUDaniqEzUbz8yFuef7KuAyvSEzJeBhNnnSvQTsYs');
