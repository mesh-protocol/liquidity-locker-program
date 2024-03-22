import * as anchor from '@coral-xyz/anchor';
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
} from '@solana/spl-token';

import { program, DEPLOYER, raydiumProgramId } from '../constants';
import { RaydiumPosition } from '../position';

async function claimFees() {
  const user = DEPLOYER.publicKey;
  const receiver = new anchor.web3.PublicKey('FiJufzNBgi8A44unCNiq1hmtnBwkEEgh9CS9aXrMr7yn');
  const raydiumPosition = await new RaydiumPosition(
    'BGWnUDaniqEzUbz8yFuef7KuAyvSEzJeBhNnnSvQTsYs'
  ).getAccounts();

  const {
    positionNftMint,
    poolState,
    personalPosition,
    protocolPosition,
    tokenMint0,
    tokenMint1,
    tokenVault0,
    tokenVault1,
    tickArrayLower,
    tickArrayUpper,
  } = raydiumPosition;

  const [locker] = anchor.web3.PublicKey.findProgramAddressSync(
    [user.toBuffer(), positionNftMint.toBuffer(), Buffer.from('locker')],
    program.programId
  );

  const memoProgramV2 = new anchor.web3.PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr');

  const nftVault = getAssociatedTokenAddressSync(positionNftMint, locker, true);
  const recipientTokenAccount0 = getAssociatedTokenAddressSync(tokenMint0, receiver, true);
  const recipientTokenAccount1 = getAssociatedTokenAddressSync(tokenMint1, receiver, true);

  const txId = await program.methods
    .claimFees()
    .accounts({
      user,
      nftMint: positionNftMint,
      locker,
      poolState,
      nftVault,
      personalPosition,
      protocolPosition,
      tokenVault0,
      tokenVault1,
      tickArrayLower,
      tickArrayUpper,
      recipientTokenAccount0,
      recipientTokenAccount1,
      vault0Mint: tokenMint0,
      vault1Mint: tokenMint1,
      memoProgram: memoProgramV2,
      raydiumClmmProgram: raydiumProgramId,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenProgram2022: TOKEN_2022_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([DEPLOYER])
    .rpc();

  console.log('-----Fees Claimed Successfully-------');
  console.log('txId => ', txId);
}

claimFees();
