import {
  Connection,
  clusterApiUrl,
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";
import { createMultisig, createMint, mintTo } from "@solana/spl-token";
import "dotenv/config";

// Підключення до devnet
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

// Завантаження ключів учасників
const signer1 = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(process.env["SECRET_KEY_1"]))
);
const signer2 = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(process.env["SECRET_KEY_2"]))
);
// Додайте ще учасників, якщо потрібно

// Створення Multisig облікового запису
const createMultisigAccount = async () => {
  const requiredSigners = 2; // Кількість підписантів, необхідних для затвердження транзакції
  const multisigAccount = await createMultisig(
    connection,
    signer1, // Платник за транзакцію
    [signer1.publicKey, signer2.publicKey], // Учасники мультипідпису
    requiredSigners
  );

  console.log("Multisig account created:", multisigAccount.toBase58());
  return multisigAccount;
};

const createToken = async (multisigAccount: PublicKey) => {
  // Створення токена з використанням Multisig як mint authority
  const mint = await createMint(
    connection,
    signer1, // Платник за транзакцію
    multisigAccount, // Multisig як mint authority
    null, // Freeze authority (може бути null)
    9 // Кількість знаків після коми
  );

  console.log("Token created with mint authority:", mint.toBase58());
  return mint;
};

const mintTokens = async (mint: PublicKey, multisigAccount: PublicKey) => {
  // Інструкція для карбування токенів
  const mintToInstruction = await mintTo(
    connection,
    signer1,
    mint,
    multisigAccount,
    [signer1, signer2],
    1000
  );
  const transaction = new Transaction().add(mintToInstruction);

  // Підписання транзакції
  await sendAndConfirmTransaction(connection, transaction, [signer1, signer2]);

  console.log("Tokens minted successfully!");
};

(async () => {
  const multisigAccount = await createMultisigAccount();
  const token = await createToken(multisigAccount);
  await mintTokens(token, multisigAccount);
})();
