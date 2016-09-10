const int TEMP_SENS = 0;
const float OFFSET = -20;

void setup() {
  Serial.begin(9600);
}

void loop() {
  int rawIn = analogRead(TEMP_SENS);
  float miliVolts = (rawIn / 1024.0) * 5000;
  float kelvin = (miliVolts / 10) + OFFSET;
  Serial.print(kelvin);
  Serial.println("degrees kelvin");

  float celsius = kelvin - 273.15;
  Serial.print(celsius);
  Serial.println("degrees Celsius");

  delay(3000);
}
