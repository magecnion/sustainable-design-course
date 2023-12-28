# kata-game-of-life

https://codingdojo.org/kata/GameOfLife/

empecé programando una célula como struct(x,y) luego viendo tu solución me di cuenta que esto genera mucho acoplcamiento con la impl del mundo, si mañan el mundo es 3d tendria que tocar la impl de la celula; tiene todo el sentido pasarle ya los vecinos a la funcion que cambia el estado de la celula, el mundo es quien sabe si la celula tiene vecinos o no

me lo imagine de otra forma

ventajas vs desventajas de mantener el state en la clase, tuve dudas

cuando fui a implementar lo que llamo test de integration me di cuenta que habia mucha logica de implementacion que el cliente de la api tenia que etner en cuenta
de primeras la api de la fn pedia hashmap pero como la primera iteracion tal pues lo limite a un vec aunque internamente es un hasmap