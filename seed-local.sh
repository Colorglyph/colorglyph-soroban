soroban config identity fund default --network local
soroban lab token wrap --asset native --network local --source default
curl -X GET "http://localhost:8000/friendbot?addr=GA55USY2TY4DEO5YFQ3KZECL2A3A5IVYVCKPB4LLTAE57TOE6PM46D7C"
curl -X GET "http://localhost:8000/friendbot?addr=GBGP5SD75TDB2ZL7JDJEFPSWDBEQRDJ4757ZXL57TOOQJSMWROT5JYKD"
curl -X GET "http://localhost:8000/friendbot?addr=GAID7BB5TASKY4JBDBQX2IVD33CUYXUPDS2O5NAVAP277PLMHFE6AO3Y"
curl -X GET "http://localhost:8000/friendbot?addr=GDKZ4O7446TNQTR3NZVJTAS7FTF6B6P2VF3B5NT2SMB2BPAF5OMIJO4S"