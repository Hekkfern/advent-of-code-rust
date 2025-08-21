# Solution explanation for AoC 2023 Day 6

Formula:

distance = velocity * time
distance = f(pressTime) = (totalTime - pressTime) * pressTime 

If we expand it, we get:

distance = -pressTime^2 + totalTime * pressTime

look for:

-pressTime^2 + totalTime * pressTime = race.distance
-pressTime^2 + totalTime * pressTime - race.distance = 0

resolve function:

-pressTime^2 + totalTime * pressTime - race.distance = 0
pressTime^2 - totalTime * pressTime + race.distance = 0

pressTime = (totalTime +- sqrt(total^2 - 4*race.distance)) / 2
