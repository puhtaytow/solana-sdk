# Frozen ABI

## AbiExample

## AbiEnumVisitor

## StableABI

Jest to opcjonalne rozwiniecie pakietu frozen-abi, pozwalajace na wykrycie zmian layoutu binarnego jako dodatek do sprawdzania zgodnosci api.

Zeby uzyc, wymagane jest zaimplementowanie `arbitrary::Arbitrary` dla implementujacego StableAbi typu.
O ile w wielu przypadkach implementacja moze byc derive(Arbitrary) o tyle nie kaz