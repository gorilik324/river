  public function bollingerBand(int $period): array
  {
    // * Get 40 days of data, need first 20 for first data point.
    $targetPrices = $this->simpleMovingAverage($period * 2)["targetPrices"];

    $upperBB = 0;
    $lowerBB = 0;
    $middleBB = 0;

    for ($i = $period; $i < count($targetPrices); $i++) {
      // * Progressivly "climb up" the arrray one value at a time
      $prices = array_slice($targetPrices, $i - $period, 20);
      $mean = $this->calcMeanOfPrices($prices);
      $standardDeviation = $this->calcStandardDeviation($prices);

      // * Get the plot point of this current target slice
      $plots = $this->calcPlotPoint($mean, $standardDeviation);

      $upperPlot = $this->roundNumber($plots["upperPlot"], 100);
      $lowerPlot = $this->roundNumber($plots["bottomPlot"], 100);

      // * When the last number is hit. Used to calculate BBtrend.
      if (count($targetPrices) - $i === 1) {
        $upperBB = $this->roundNumber($upperPlot, 100);
        $middleBB = $this->roundNumber($mean, 100);
        $lowerBB = $this->roundNumber($lowerPlot, 100);
      }
    }

    return [
      "upperBB" => $upperBB,
      "middleBB" => $middleBB,
      "lowerBB" => $lowerBB,
    ];
  }
