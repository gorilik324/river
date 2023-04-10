 public function exponentialMovingAverage(int $days, ?array $data = null): int | float
  {
    $closePricesInTimeframe = $data ?? [...$this->closes];

    // * Need twice as many data points, half go to the initial SMA calculation, which is the first EMA value.
    $sliceLength = $days * 2;
    $targetPrices = array_slice($closePricesInTimeframe, -$sliceLength);

    // * Calc the SMA from half the data points.
    $simpleAveragePriceRange = array_slice($targetPrices, 0, $days);
    $simpleMovingAverage = $this->calcMeanOfPrices($simpleAveragePriceRange);

    // * Put the SMA of the previous days into the first index of the EMA data set.
    $exponentialAverageTargetRange = array_slice($targetPrices, -$days + 1);
    array_unshift($exponentialAverageTargetRange, $simpleMovingAverage);

    $ema = array_reduce($exponentialAverageTargetRange, function ($prev, $curr) use ($days) {
      $multiplier = 2 / ($days + 1);
      return ($curr - $prev) * $multiplier + $prev;
    }, $exponentialAverageTargetRange[0]);

    return $this->roundNumber($ema, 100);
  }
