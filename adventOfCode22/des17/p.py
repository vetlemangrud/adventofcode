jets = [1 if l == '>' else -1 for l in open('input').read().strip()]
well = set([-1j,1-1j,2-1j,3-1j,4-1j,5-1j,6-1j])
highest = -1
piece_index = 4
jet_index = len(jets)-1
p1, p2 = 0, 0
history = dict()
limit = 1_000_000_000_000
pieces = [[2+0j,3+0j,4+0j,5+0j], 
          [3+0j,2+1j,3+1j,4+1j,3+2j],
          [2+0j,3+0j,4+0j,4+1j,4+2j],
          [2+0j,2+1j,2+2j,2+3j],
          [2+0j,3+0j,2+1j,3+1j]]   

for n in range(limit):
    
    seen = (piece_index, jet_index)
    if seen in history:
        period = n - history[seen][0]
        if n % period == limit % period:
            p2 = history[seen][1] + (highest+1 - history[seen][1])*(((limit-n)//period)+1)
            break
    else:
        history[seen] = n, highest+1

    if n == 2022:
        p1 = highest+1 

    piece_index = 0 if piece_index == 4 else piece_index+1
    piece = [x + (highest+4)*1j for x in pieces[piece_index]]  
    
    while True:
        jet_index = 0 if jet_index == len(jets)-1 else jet_index + 1
        jet = jets[jet_index]
        piece = [x+jet for x in piece]
        if any([(x.real < 0) or (x in well) or (x.real > 6) for x in piece]):
            piece = [x-jet for x in piece]
        piece = [x-1j for x in piece]
        if any([x in well for x in piece]):
            well |= set([x+1j for x in piece])
            highest = max(highest, int(piece[-1].imag)+1)
            break

print(p1,p2)    
