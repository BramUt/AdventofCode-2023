from dataclasses import dataclass

@dataclass
class Map:
    dest_start: int
    source_start: int
    length: int

    def calc_value(self, value):
        return self.source_start + (value - self.dest_start)

def check_map(value, map: Map, value_name):
    print(f"{value_name:<11}: {value: 11}")
    if value < map.source_start or value >= map.source_start + map.length:
        print(f"{value_name} doesn't check out.")

location = 59_370_572
print(f"Location   : {location: 11}")


humid_map = Map(dest_start=0, source_start=180532143, length=151140540)
humidity = humid_map.calc_value(location)
check_map(humidity, humid_map, "Humidity")

temp_map = Map(dest_start = 157942811, source_start = 3887783726, length = 359543023)
temperature = temp_map.calc_value(humidity)
check_map(temperature, temp_map, "Temperature")

light_map = Map(dest_start=3810633876, source_start=2864755793, length=177520878)
light = light_map.calc_value(temperature)
check_map(light, light_map, "Light")

water_map = Map(dest_start=2903664303, source_start=775683406, length=169307196)
water = water_map.calc_value(light)
check_map(water, water_map, "Water")

fert_map = Map(dest_start=895884650, source_start=0, length=159434487)
fert = fert_map.calc_value(water)
check_map(fert, fert_map, "Fertilizer")

# (fertilizer 0 maps to soil 0)
print(f"Soil       : {0:11} ")

seed_map = Map(dest_start=0, source_start=1623310249, length=453595079)
seed = seed_map.calc_value(fert)
check_map(seed, seed_map, "Seed")