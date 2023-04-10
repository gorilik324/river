  public function MACD(int $period)
  {
    $macdLine = [];

    for ($i = 0; $i <= $period; $i++) {
      // * Remove the last element in array on each loop through.
      $sliceLength = count($this->closes) - $i;

      $targetPrices = array_slice($this->closes, 0, $sliceLength);

      $shortPeriod = $this->exponentialMovingAverage(12, $targetPrices);

      $longPeriod = $this->exponentialMovingAverage(26, $targetPrices);

      $diff = $shortPeriod - $longPeriod;

      array_unshift($macdLine, $this->roundNumber($diff, 100));
    }

    $signal_line = $this->exponentialMovingAverage($period / 2, $macdLine);
    return [
      "fast_line" => array_slice($macdLine, -3),
      "slow_line" => $signal_line
    ];
  }
