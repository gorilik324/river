  public function simpleMovingAverage(int $days): array
  {
    $closePricesInTimeframe = [...$this->closes];

    // * Trim data to be the correct sized timeframe. 10 days, 50 days, etc..
    $targetPrices = array_splice($closePricesInTimeframe, -$days);

    $mean = $this->roundNumber($this->calcMeanOfPrices($targetPrices), 100);

    return ["mean" => $this->roundNumber($mean, 100), "targetPrices" => $targetPrices];
  }
