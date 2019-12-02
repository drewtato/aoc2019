from PIL import Image

with open('notes/day02grid.txt') as f:
	grid = [[int(n) for n in row.split()] for row in f]

# lowest = min(min(grid))
# highest = max(max(grid))

# print(lowest, highest)

for y in range(100):
	for x in range(100):
		val = grid[y][x]
		if val == 19690720:
			answerpx = (x,y)
		r = val
		g = val % 100
		b = 0
		grid[y][x] = (r,g,b)

# for row in grid:
# 	print(row)

def idx(i):
	return lambda thing: thing[i]
	
rhigh = max((n[0] for row in grid for n in row))
rlow = min((n[0] for row in grid for n in row))

for y in range(100):
	for x in range(100):
		r,g,b = grid[y][x]
		r = (r - rlow) * 255 // (rhigh - rlow)
		g = g * 255 // 100
		grid[y][x] = (r,g,b)

grid = [px for row in grid for px in row]

i = Image.new('RGB', (100,100))
i.putdata(grid)
i.putpixel(answerpx, (255,255,255))
i = i.resize((500,500))
i.save('notes/day02.png')