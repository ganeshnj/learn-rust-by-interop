using HttpClientInterop;
using System;
using System.Collections.Generic;
using System.Text.Json;

var client = new HttpClient();

var request = new HttpRequest
{
    Host = "localhost:5104",
    Path = "/WeatherForecast"
};
var response = client.Get(request);

Console.WriteLine(response.Status);

var forecasts = JsonSerializer.Deserialize<List<WeatherForecast>>(response.Body);
Console.WriteLine($"Date\t\t\tC\tF\tSummary");
foreach (var forecast in forecasts)
{
    Console.WriteLine($"{forecast.Date}\t{forecast.TemperatureC}\t{forecast.TemperatureF}\t{forecast.Summary}");
}