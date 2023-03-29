#include<bits/stdc++.h>
using namespace std;
 
int main() {
    ios_base::sync_with_stdio(false); cin.tie(NULL);
    int n;
    cin>>n;
 
    long long arr[n],ans;
    ans=0;
    for (int i = 0; i < n; i++)
    {
        cin>>arr[i];
    }
    for (int i = 0; i < n-1; i++)
    {
        if(arr[i]>arr[i+1]) {
            ans+=arr[i]-arr[i+1];
            arr[i+1]=arr[i];
        }
    }
    cout<<ans;
    
    return 0;
}