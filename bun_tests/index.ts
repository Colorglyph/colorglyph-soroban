import { xdr, Address } from '@stellar/stellar-sdk'

const contract = Address.fromString('CARZSHD6BLSLB5ENFR76QI4VNJ2XUHXEDCRG77VMLOAICRG7MZTIZPA7')

console.log(
    contract.toBuffer()
);

console.log(
    xdr.ScVal.scvVoid().toXDR('base64')
);

console.log(
    Address.fromString('GBGP5SD75TDB2ZL7JDJEFPSWDBEQRDJ4757ZXL57TOOQJSMWROT5JYKD').toScVal().toXDR('base64')
);