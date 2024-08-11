import { Keypair } from "@solana/web3.js";
import * as readline from "readline";

// Function for generating a new public key
function generatePublicKeyWithPrefix(prefix: string): {
  publicKey: string;
  attempts: number;
} {
  let attempts = 0;
  let publicKey = "";
  const normalizedPrefix = prefix.toLowerCase();

  do {
    const keypair = Keypair.generate();
    publicKey = keypair.publicKey.toString().toLowerCase();
    attempts++;
  } while (!publicKey.startsWith(normalizedPrefix));

  return { publicKey, attempts };
}

// Function for measuring the execution time
function measureExecutionTime(fn: () => void): number {
  const start = Date.now();
  fn();
  const end = Date.now();
  return end - start;
}

// Enter the required prefix from the console
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

rl.question("Enter the desired prefix (case-insensitive): ", (prefix) => {
  const timeTaken = measureExecutionTime(() => {
    const result = generatePublicKeyWithPrefix(prefix);
    console.log(`Generated public key: ${result.publicKey}`);
    console.log(`Attempts: ${result.attempts}`);
  });

  console.log(`Time taken: ${timeTaken} ms`);
  rl.close();
});
