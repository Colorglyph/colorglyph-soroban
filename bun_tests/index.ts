import { Address } from '@stellar/stellar-sdk'

const contract = Address.fromString('CARZSHD6BLSLB5ENFR76QI4VNJ2XUHXEDCRG77VMLOAICRG7MZTIZPA7')

console.log(
    contract.toBuffer()
);