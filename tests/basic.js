const anchor = require("@project-serum/anchor");
const { assert } = require("chai");

describe("basic", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Basic;
  const remainingAccount = anchor.web3.Keypair.generate();
  const invalidRemainingAccount = anchor.web3.Keypair.generate();

  before(async () => {
    // Create a valid remaining account
    await program.rpc.createAccount({
      accounts: {
        myAccount: remainingAccount.publicKey,
        authority: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [remainingAccount],
    });
  });

  it("should create when remaining account is valid", async () => {
    // arrange
    const newAccount = anchor.web3.Keypair.generate();
    // act
    await program.rpc.createAccountWithRemainingAccount({
      accounts: {
        myAccount: newAccount.publicKey,
        authority: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [newAccount],
      remainingAccounts: [
        {
          pubkey: remainingAccount.publicKey,
          isWritable: false,
          isSigner: false,
        },
      ],
    });
  });

  it("should fail when remaining accounts is empty", async () => {
    // arrange
    const newAccount = anchor.web3.Keypair.generate();
    let error;
    // act
    try {
      await program.rpc.createAccountWithRemainingAccount({
        accounts: {
          myAccount: newAccount.publicKey,
          authority: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [newAccount],
        remainingAccounts: [],
      });
    } catch (err) {
      error = err;
    }
    // assert
    assert.equal(error.code, 300);
  });

  it("should fail when remaining account is invalid", async () => {
    // arrange
    const newAccount = anchor.web3.Keypair.generate();
    let error;
    // act
    try {
      await program.rpc.createAccountWithRemainingAccount({
        accounts: {
          myAccount: newAccount.publicKey,
          authority: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [newAccount],
        remainingAccounts: [
          {
            pubkey: invalidRemainingAccount.publicKey,
            isWritable: false,
            isSigner: false,
          },
        ],
      });
    } catch (err) {
      error = err;
    }
    // assert
    assert.equal(error.code, 301);
  });
});
