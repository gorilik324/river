  public function stochastic(int $period)
  {
    $closePricesInTimeframe = [...$this->closes];

    // * Trim data to be the correct sized timeframe. 10 days, 50 days, etc..
    $plotLength = 5;
    $sliceLength = $period + $plotLength;
    $targetPrices = array_splice($closePricesInTimeframe, -$sliceLength);

    $kLine = [];
    $dLine = [];

    for ($i = 0; $i <= $plotLength; $i++) {
      // * Progressivly "climb up" the arrray one value at a time
      $prices = array_slice($targetPrices, $i, $period);

      // * Last close price
      $recentPrice = end($prices);
      $lowestPrice = min($prices);
      $highestPrice = max($prices);

      $x = abs($recentPrice - $lowestPrice);
      $a = abs($highestPrice - $lowestPrice);
      $b = $x / $a;

      $k = $b * 100;

      array_push($kLine, $this->roundNumber($k, 100));
    }


    for ($i = 3; $i <= count($kLine); $i++) {
      // * Progressivly "climb up" the arrray one value at a time
      $prices = array_slice($kLine, $i - 3, 3);

      $sumOfPrices = array_sum($prices);

      $mean = $sumOfPrices / 3;

      array_push($dLine, $this->roundNumber($mean, 100));
    }

    return ["k_line" => $kLine, "d_line" => $dLine];
  }
