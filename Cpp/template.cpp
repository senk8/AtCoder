#include <bits/stdc++.h>
using namespace std;
//#include <atcoder/all>
//using namespace atcoder;
#define rep(i,start,n) for (int i = start; i < (int)(n); i++)
#define judge(cond1,cond2) cout<<((cond1==cond2)?"Yes":"No")<<endl;
#define yesno(expr) cout<<((expr)?"Yes":"No")<<endl;
#define dout(expr) cout << fixed << setprecision(10) << expr << endl;
#define Sort(a) sort(a.begin(),a.end())
#define all(a)  (a).begin(),(a).end()
#define elif else if 
#define out(a) cout<<a<<endl
using ll = long long;
using ull = unsigned long long;
using pll = pair<ll,ll>;
using vvll = vector<vector<ll>>;
using vll = vector<ll>;
using vstr = vector<string>;
using Graph = vector<vector<Edge>>;
const ll MOD=1000000007;
vll dx{0,1,0,-1};
vll dy{1,0,-1,0};
ll a,b,c,d,x,y,z,i,j,k,m,n,h,w,res,sum;
string s,t,ans;
vll vec1,vec2,vec3;
vstr maze;
bool flag;


struct Edge {
    int to;
    ll w;
    Edge(int to,ll w):to(to),w(w){}
};




bool IsOk(ll index,ll key) {
    if (vec1[index] >= key) return true;
    else return false;
}


template <class type> void PrintVector(vector<type> vec){
    stringstream ss;
    n=vec.size();
    for(i=0;i<n;i++)
    {
        ss<<vec[i]<<" ";
        if(i!=n-1)ss<<" ";
    }
    cout<<ss.str()<<endl;
}

template <class type> void InputVector(vector<type>& vec){
    for(i=0;i<vec.size();i++)cin>>vec[i];
}

ull GetDigit(ull num){
    return std::to_string(num).length();
}


//--------------------------------------------------------------

struct UnionFind {
    vector<int> par; // par[i]:iの親の番号　(例) par[3] = 2 : 3の親が2

    UnionFind(int N) : par(N) { //最初は全てが根であるとして初期化
        for(int i = 0; i < N; i++) par[i] = i;
    }

    int Root(int x) { // データxが属する木の根を再帰で得る：root(x) = {xの木の根}
        if (par[x] == x) return x;
        return par[x] = Root(par[x]);
    }

    void Unite(int x, int y) { // xとyの木を併合
        int rx = Root(x); //xの根をrx
        int ry = Root(y); //yの根をry
        if (rx == ry) return; //xとyの根が同じ(=同じ木にある)時はそのまま
        par[rx] = ry; //xとyの根が同じでない(=同じ木にない)時：xの根rxをyの根ryにつける
    }

    bool Same(int x, int y) { // 2つのデータx, yが属する木が同じならtrueを返す
        int rx = Root(x);
        int ry = Root(y);
        return rx == ry;
    }
};

//--------------------------------------------------------------

void BFS()
{
    queue<pll> que;
    vvll distance(10,vll(10,-1));
    que.push(make_pair(0,0));
    distance[0][0]=0;

    while(!que.empty())
    {
       auto v=que.front();
       que.pop();
       for(ll i=0;i<4;i++)
       {
           ll x=v.first+dx[i];
           ll y=v.second+dy[i];
           if(x<0 || 50<=x)continue;
           if(y<0 || 50<=y)continue;
           if(distance[x][y]==-1)
           {
               que.push(pll(x,y));
               distance[x][y]=distance[v.first][v.second]+1;
           }
       }
    }
}

vll AccumlateSum(vll vec)
{
    vll asum=vll(n+1,0);
    asum[0]=0;
    rep(i,0,n)asum[i+1]=asum[i]+vec[i];
    return asum;
}

void BitsSearch(){
    sum=0;
    rep(bit,0,1<<n){
        rep(bitlen,0,n){
            if((bit>>bitlen) & 1)sum++;
        }
   }
}

ll BinarySearch(ll key){
    ll ng=-1;
    ll ok=vec1.size();
    while(1<abs(ok-ng)){
        ll mid=(ok+ng)/2;
        if(IsOk(mid,key))ok=mid;
        else ng=mid;
    }
    return ok;
}

void Syakutori(){
    ll res=0;
    ll sum=0;
    ll right=0;
    rep(left,0,n){
        while(right < n && sum < k){
            sum+=vec1[right];
            ++right;
        }
        res += n-right+1;       //rightまででk以上になっている。n-rightがleftを先頭とした部分列の全パターン
        if(left==right)right++; //例えば0==0だ永遠に進まず困る。elseでいいのは==ならそれはrightを一回も進めていないときだけだからsumも増えていない
        else sum -= vec1[left]; //leftを合計からひかないと、カーソルを進めてもleftが和に含まれてしまう
    }
}

//------------------ Graph Search-------------------------------


const ll INF = 1LL << 60;

vll Dijkstra(Graph G,ll start){
    ll n = G.size();
    vector<bool> used(n,false);
    vll distance(n,INF);
    distance[start]=0;

    for(ll i=0;i<n;i++){
        ll min_dist = INF;
        ll min_v = -1;

        // 使用していなくて、経路が存在するパスを探して,miv_vとする。
        for(ll v=0;v<n;++v){
            if(!used[v] && distance[v] < min_dist){
                min_dist = distance[v];
                min_v = v;
            }
        }
        if(min_v == -1)break;


        // min_vから行ける頂点を全て緩和する。
        for(auto e:G[min_v]){
            distance[e.to]=min(distance[e.to],distance[min_v]+e.w);
        }
        used[min_v] = true;
    }
    return distance;
}


//--------------------------------------------------------------

int Factorial(int n)
{
    int fact = n;

    if (n <= 0) return 1;

    while (--n)
        fact *= n;

    return fact;
}

int Combination(int n, int r)
{
    if (r <= 0) return 1;
    return Factorial(n) / (Factorial(r) * Factorial(n - r));
}

vll EnumDivisor(ll num)
{
    set<long> tmp;
    for(long d=1;d*d<=n;d++){
        if(n%d==0){
            tmp.insert(d);
            tmp.insert(n/d);
        }
    }
    vector<ll> res(tmp.begin(), tmp.end());
    return res;
}


ll GCD(ll a, ll b) {
    if (b == 0) return a;
    else return GCD(b, a % b);
}

ll LCM(ll a, ll b)
{
   return (a * b) / GCD(a, b);
}

ll SeqSum(ll a,ll n,ll d){
    return n*(2*a+(n-1)*d)/2;
}

bool OnQuad(ll x1,ll x2,ll y1,ll y2,ll x,ll y)
{
    return (x2-x1)*(y-y1)==(y2-y1)*(x-x1);
}

vector<pll> PrimeFactorize(ll num)
{
    vector<pair<ll,ll>> res;
    for(ll p=2;p*p<=num;++p){
        if(num%p!=0)continue;
        ll ex=0;
        while(num%p==0){
            ++ex;
            num/=p;
        }
        res.push_back({p,ex});
    }
    if(num!=1)res.push_back({num,1});
    return res;
}

//--------------------------------------------------------------



template<ll MOD> struct Fp {
    ll val;
    constexpr Fp(ll v = 0) noexcept : val(v % MOD) {
        if (val < 0) val += MOD;
    }
    constexpr ll getmod() { return MOD; }
    constexpr Fp operator - () const noexcept { //符号
        return val ? Fp(MOD - val) : Fp(0);
        //return val ? MOD - val : 0; これじゃだめ？
    }
    constexpr Fp operator + (const Fp& r) const noexcept { return Fp(this->val) += r; }//ここでthisなのは？->コピーコンストラクタが自動生成されているから ここでFp()なのは自身の値は変わらないから
    constexpr Fp operator - (const Fp& r) const noexcept { return Fp(this->val) -= r; }
    constexpr Fp operator * (const Fp& r) const noexcept { return Fp(this->val) *= r; }
    constexpr Fp operator / (const Fp& r) const noexcept { return Fp(this->val) /= r; }
    constexpr Fp& operator += (const Fp& r) noexcept {
        val += r.val;
        if (val >= MOD) val -= MOD;
        return *this;
    }
    constexpr Fp& operator -= (const Fp& r) noexcept {
        val -= r.val;
        if (val < 0) val += MOD;
        return *this;
    }
    constexpr Fp& operator *= (const Fp& r) noexcept {
        val = val * r.val % MOD;
        return *this;
    }
    constexpr Fp& operator /= (const Fp& r) noexcept {
        ll a = r.val, b = MOD, u = 1, v = 0;
        while (b) {
            ll t = a / b;
            a -= t * b; swap(a, b);
            u -= t * v; swap(u, v);
        }
        val = val * u % MOD;
        if (val < 0) val += MOD;
        return *this;
    }
    constexpr bool operator == (const Fp& r) const noexcept {
        return this->val == r.val;
    }
    constexpr bool operator != (const Fp& r) const noexcept {
        return this->val != r.val;
    }
    friend constexpr ostream& operator << (ostream &os, const Fp<MOD>& x) noexcept {
        return os << x.val;
    }
    friend constexpr Fp<MOD> modpow(const Fp<MOD> &a, ll n) noexcept {
        if (n == 0) return 1;
        auto t = modpow(a, n / 2);
        t = t * t;
        if (n & 1) t = t * a;//1bit目が1=奇数,1ならここは2になる。
        return t;
    }
};

const int MAX = 3000000;
long long fac[MAX], finv[MAX], inv[MAX];

void COMinit() {
    fac[0] = fac[1] = 1;
    finv[0] = finv[1] = 1;
    inv[1] = 1;
    for (int i = 2; i < MAX; i++){
        fac[i] = fac[i - 1] * i % MOD;
        inv[i] = MOD - inv[MOD%i] * (MOD / i) % MOD;
        finv[i] = finv[i - 1] * inv[i] % MOD;
    }
}

long long COM(int n, int k){
    if (n < k) return 0;
    if (n < 0 || k < 0) return 0;
    return fac[n] * (finv[k] * finv[n - k] % MOD) % MOD;
}

using mint = Fp<MOD>;


ll BinPow(ll a, ll b) {
	ll res = 1;
	while (b != 0) {
		if (b % 2 == 1) {
			res = res * a % MOD;
		}
		a = a * a % MOD;
		b /= 2;
	}
	return res;
}

//--------------------------------------------------------------

void in(ll which)
{
    switch (which)
    {
        case 0:
            cin>>n;break;
        case 1:
            cin>>s;break;
        case 2:
            cin>>n>>m;break;
        case 3:
            cin>>s>>t;break;
        case 4:
            cin>>x>>y>>z;break;
        case 5:
            cin>>a>>b>>c>>d;break;
        case 6:
            cin>>n>>s;break;
        case 7:
            cin>>n;
            vec1=vll(n);
            rep(i,0,n)cin>>vec1[i];
            break;
        case 8:
            cin>>h>>w;
            maze=vstr(h);
            rep(i,0,w)cin>>maze[i];
            break;
        default:
            break;
    }
}

//----------------------------------------

void Answer()
{
    in(0);
    out(res);
}

int main() 
{
    Answer();
    return 0;
}
