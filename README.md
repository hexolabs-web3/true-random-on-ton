# RandomTON Proof-of-Work Service.

This is an open source implementation of a Proof-of-Work Service for RandomTON.

---

## Installation.

```
git clone git@github.com:hexolabs-web3/true-random-on-ton.git
cd true-random-on-ton
docker compose up --build -d
curl -X GET https://localhost:3222/api/vrf/sk/new
```

---

## Integration guide.

### **Step 0: Public Key & Secret Key Setup**  
**UI Inputs**:  
- `Public Key` (user-provided)  
- `Secret Key` (read-only, masked)  

**API Request**:  
1. **Convert Public Key to HEX**  
   - **Endpoint**: `POST /api/utils/hex`

---

### **Step 1: Generate Alpha (Step 1)**  
**UI Inputs**:  
- `rng` (random number generator value)  
- `time` (timestamp)  

**API Requests**:  
1. **Convert `rng` to HEX**:  
   - **Endpoint**: `POST /api/utils/hex`

2. **Convert `time` to HEX**:  
   - **Endpoint**: `POST /api/utils/hex`

3. **Generate Alpha (SHA256 Hash)**:  
   - **Endpoint**: `POST /api/utils/sha256`

4. **Convert Alpha to Integer**:  
   - **Endpoint**: `POST /api/utils/int`

---

### **Step 2: Generate Pi (Step 2)**  
**UI Inputs**:  
- `Gamma` (elliptic curve point)  
- `c` (challenge value)  
- `s` (proof value)  

**API Requests**:  
1. **Convert `Gamma` to HEX**:  
   - **Endpoint**: `POST /api/utils/hex`

2. **Convert `c` to HEX**:  
   - **Endpoint**: `POST /api/utils/hex`

3. **Convert `s` to HEX**:  
   - **Endpoint**: `POST /api/utils/hex`

4. **Generate Pi (Concatenate Gamma, c, s HEX values)**:  
   - Used for verification in Step 4.
   - **No Requests**: simple concatenation of strings.
     ```js
     return GammaHex + cHex + sHex;
     ```
3. **Convert `Pi` to INT**:  
   - **Endpoint**: `POST /api/utils/int`

---

### **Step 3: Verify VRF Proof (Step 3)**  
1. **Verify VRF Proof and receive `beta`: 
- **Endpoint**: `POST /api/vrf/verify`  

**Post-Processing**:  
- Split `beta` into two 64-byte values:  
  - `h1 = beta.substring(0, 64)`  
  - `h2 = beta.substring(64, 128)`]
- Print `h1` and `h2` out.
    
2. **Convert `h1` to INT**:  
   - **Endpoint**: `POST /api/utils/int`

3. **Convert `h2` to INT**:  
   - **Endpoint**: `POST /api/utils/int`

---

### **Step 4: Generate Random Seed (Step 4)**  
1. **Hash `beta` to create a random seed**:  
   - **Endpoint**: `POST /api/utils/sha256`

2. **Convert seed to integer**:  
   - **Endpoint**: `POST /api/utils/int`

---

### **Step 5: Generate Random Values**  
**UI Inputs**:  
- `Number of Tickets` (limit)  
- `Number of Winners` (iterations)  

**API Request**:  
- **Endpoint**: `POST /api/random`  

- **Example of results**
  ```js
  x.data.results.forEach( (x, i) =>
  {
    console.log(`Winner #${i + 1}: ticket ${x["ticket_number"]}, new seed: ${new_seed}`);
  } );
  ```

---
