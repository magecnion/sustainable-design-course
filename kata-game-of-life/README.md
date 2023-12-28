# kata-game-of-life

https://codingdojo.org/kata/GameOfLife/

empecé programando una célula como struct(x,y) luego viendo tu solución me di cuenta que esto genera mucho acoplcamiento con la impl del mundo, si mañan el mundo es 3d tendria que tocar la impl de la celula; tiene todo el sentido pasarle ya los vecinos a la funcion que cambia el estado de la celula, el mundo es quien sabe si la celula tiene vecinos o no

me lo imagine de otra forma