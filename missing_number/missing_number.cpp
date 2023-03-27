#include<bits/stdc++.h>
using namespace std;
 
int main() {
    ios_base::sync_with_stdio(false); cin.tie(NULL);
    long long n,num;
    cin>>n;
    long long sum=n*(n+1)/2;
    while(n>1){
        cin>>num;
        sum-=num;
        n--;
    }
    cout<<sum;
    return 0;
}