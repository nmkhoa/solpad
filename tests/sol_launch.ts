import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SolLaunch } from '../target/types/sol_launch';
import { assert } from 'chai';
import moment from 'moment';

const sleep = (seconds) => {
    return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
};

describe('sol_launch', () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const provider = anchor.AnchorProvider.env();

    const program = anchor.workspace.SolLaunch as Program<SolLaunch>;

    const owner = anchor.web3.Keypair.generate();
    const creator = anchor.web3.Keypair.generate();
    const currency = anchor.web3.Keypair.generate();
    const token = anchor.web3.Keypair.generate();
    const signer = anchor.web3.Keypair.generate();

    const eventParser = new anchor.EventParser(program.programId, program.coder);

    const CONFIG_ACCOUNT_SEED = 'ido_platform_seed';

    const [configAccount] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(CONFIG_ACCOUNT_SEED)], program.programId);

    before(async () => {
        await Promise.all(
            [owner.publicKey, creator.publicKey].map(async (address) => {
                // await provider.connection.requestAirdrop(
                //   address,
                //   2 * 10 ** 9
                // );
                await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(address, 1_000 * 10 ** 9));
            })
        );
    });

    it('Is initialized!', async () => {
        // Add your test here.
        const tx = await program.methods.initialize(owner.publicKey, creator.publicKey).rpc();
        console.log('Your transaction signature', tx);

        const configInfo = await program.account.configAccount.fetch(configAccount);

        assert.equal(configInfo.owner.toBase58(), owner.publicKey.toBase58());
        assert.equal(configInfo.creator.toBase58(), creator.publicKey.toBase58());

        await sleep(1);
        const parsedTransaction = await provider.connection.getParsedTransaction(tx, {
            maxSupportedTransactionVersion: 0,
            commitment: 'confirmed',
        });
        console.log(parsedTransaction.meta.logMessages);
        const events = eventParser.parseLogs(parsedTransaction?.meta?.logMessages);
        let parsedEvents = [];
        for (const event of events) {
            parsedEvents.push(event);
        }
    });

    const startTime = Math.floor(moment().add(10, 'seconds').valueOf() / 1000);
    const endTime = Math.floor(moment().add(50, 'seconds').valueOf() / 1000);
    const claimTime = Math.floor(moment().add(50, 'seconds').valueOf() / 1000);
    const tokensForSale = 1_000_000;
    const tokenDecimnals = 6;
    const tokenRate = 0.1;
    const decimals = 3;

    describe('Create pool', async () => {
        it('Should revert if create pool by account is not a creator', async () => {
            try {
                await program.methods
                    .creatorCreatePool(
                        new anchor.BN(startTime),
                        new anchor.BN(endTime),
                        new anchor.BN(claimTime),
                        new anchor.BN(tokensForSale).mul(new anchor.BN(10).pow(new anchor.BN(tokenDecimnals))),
                        tokenDecimnals,
                        new anchor.BN(tokenRate).mul(new anchor.BN(10).pow(new anchor.BN(decimals))),
                        decimals,
                        currency.publicKey,
                        token.publicKey,
                        signer.publicKey
                    )
                    .accounts({ signer: owner.publicKey })
                    .signers([owner])
                    .rpc();
                assert.equal('Should revert but it didnt', '');
            } catch (error) {
                console.log(error);
                assert.equal(error.error.errorCode.code, 'Unauthorized');
                assert.equal(error.error.errorMessage, 'Unauthorized');
            }
        });
        it('Should revert if invalid time', () => {});
        it('Should create pool successfull', () => {});
    });
});
