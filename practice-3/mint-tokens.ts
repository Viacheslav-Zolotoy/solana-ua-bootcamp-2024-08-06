import { mintTo } from "@solana/spl-token";
import { Connection, PublicKey, Transaction } from "@solana/web3.js";

const mintTokens = async (
  mint: PublicKey,
  multisigAccount: PublicKey,
  connection: Connection,
  signer1: any,
  signer2: any
) => {
  const transaction = new Transaction();

  // Інструкція для карбування токенів
  const mintToInstruction = mintTo(
    connection,
    signer1, // Платник
    mint,
    multisigAccount, // Multisig account
    [signer1, signer2], // Підписанти
    1000 // Кількість токенів для карбування
  );

  transaction.add(mintToInstruction);

  // Підписання транзакції
  await sendAndConfirmTransaction(connection, transaction, [signer1, signer2]);

  console.log("Tokens minted successfully!");
};

(async () => {
  const multisigAccount = await createMultisigAccount();
  const token = await createToken(multisigAccount);
  await mintTokens(token, multisigAccount);
})();
