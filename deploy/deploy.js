
const { LCDClient, MsgStoreCode, MnemonicKey, isTxError, MsgInstantiateContract, MsgExecuteContract } = require('@terra-money/terra.js');
const fs = require('fs')
require('dotenv').config();

// test1 key from localterra accounts
const mk = new MnemonicKey({
    mnemonic: process.env.MNEMONIC
})

// connect to localterra
const terra = new LCDClient({
    URL: 'http://localhost:1317/',
    chainID: 'localterra'
});
const wallet = terra.wallet(mk);

async function main() {


    const code_id_nft = await storeCode('rest_nft_base');

    const init_nft_msg =
    {
        "name": "Astro Herro",
        "symbol": "Astro",
        "minter": "terra1rp0wtz8jcxcet6gvjrjtjklvt30en6psgsu0yy",
        "token_supply": 3333
    }

    const nft_contract_address = await initContract(code_id_nft, init_nft_msg)
    console.log("Deloy contract nft at address " + nft_contract_address)

    

}

main()

async function storeCode(path) {
    const storeCode = new MsgStoreCode(
        wallet.key.accAddress,
        fs.readFileSync(`./artifacts/${path}-aarch64.wasm`).toString('base64')
    );
    const storeCodeTx = await wallet.createAndSignTx({
        msgs: [storeCode],
    });
    const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);

    if (isTxError(storeCodeTxResult)) {
        throw new Error(
            `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
        );
    }

    const {
        store_code: { code_id },
    } = storeCodeTxResult.logs[0].eventsByType;
    console.log(`Store contract ${path} with code id = ${code_id}`)

    return code_id;

}

async function initContract(code_id, init_msg) {
    //init 
    const instantiate = new MsgInstantiateContract(
        wallet.key.accAddress,
        wallet.key.accAddress,
        code_id[0], // code ID
        init_msg
        // { uluna: 10000000, ukrw: 1000000 }
    );

    const instantiateTx = await wallet.createAndSignTx({
        msgs: [instantiate],
    });

    const instantiateTxResult = await terra.tx.broadcast(instantiateTx);

    const {
        instantiate_contract: { contract_address },
    } = instantiateTxResult.logs[0].eventsByType;

    return contract_address
}

async function execute(message, contract_address, coin) {
    const msg = new MsgExecuteContract(
        wallet.key.accAddress,
        contract_address.toString(),
        message,
        coin

    )
    const execute = await wallet.createAndSignTx({
        msgs: [msg]
    })
    const result = await terra.tx.broadcast(execute);
    return result.txhash
}
