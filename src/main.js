import * as nearAPI from "near-api-js";
import $ from "jquery";
const X_CLASS = 'x'
const CIRCLE_CLASS = 'circle'
const { connect, keyStores, WalletConnection } = nearAPI;
const helloMessageTextElement = document.querySelector('[data-hello-message-text]')
const cellElements = $('.cell');
const config = {
  networkId: "testnet",
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
};
let near;
let wallet;
let gameDetail;
let contract;
let gameId;

async function near_connect () {
  loading(false);
  console.log('connect')
  near = await connect(config);
  wallet = new WalletConnection(near, 'my-app');
  // If not signed in redirect to the NEAR wallet to sign in
  // keys will be stored in the BrowserLocalStorageKeyStore

  console.log('wallet.account()', wallet.account())
  if (wallet.account().accountId) {
    $('#HelloMessage').css({display: 'none'});
    $('.account_name').text(`Hello ${wallet.account().accountId}!`)
    contract = new nearAPI.Contract(
      wallet.account(), // the account object that is connecting
      "inti_demo.testnet",
      {
        // name of contract you're connecting to
        viewMethods: ["contract_metadata", "get_current_game", "get_game_detail"], // view methods do not change state but usually return a value
        changeMethods: ["play", "new_game"], // change methods modify state
        sender: wallet.account(), // account object to initialize and sign transactions.
      }
    );
    loading(true);
    gameId = await contract.get_current_game({player_id: wallet.account().accountId});
    console.log('gameId', gameId);
    await updateGame();
    loading(false);
  } else {
    $('#HelloMessage').css({display: 'flex'});
    $('.hello_text').text(`Please connect to continue!`)
  }
}

// $('#connetWallet').click(function () {
//
// });

async function updateGame() {
  if (gameId) {
    gameDetail = await contract.get_game_detail({game_id: gameId});
    console.log('gameDetail', gameDetail);
    if (gameDetail.player_turn == 1) {
      if (wallet.account().accountId == gameDetail.player1) {
        $('.turn').text(`Your turn ...`);
      } else {
        $('.turn').text(`${gameDetail.player1} turn ...`);
      }
    } else {
      if (wallet.account().accountId == gameDetail.player2) {
        $('.turn').text(`Your turn ...`);
      } else {
        $('.turn').text(`${gameDetail.player2} turn ...`);
      }
    }
    $('.game_detail').text(JSON.stringify(gameDetail, null, 2));
    let main = gameDetail.game;
    for (let i = 0; i< 9 ; i++) {
      console.log('main[i]', main[i])
      $(cellElements[i]).removeClass(X_CLASS)
      $(cellElements[i]).removeClass(CIRCLE_CLASS)
      if (main[i] === 1) {
        $(cellElements[i]).addClass(X_CLASS)
      }
      if (main[i] === 2) {
        $(cellElements[i]).addClass(CIRCLE_CLASS)
      }
    }
    console.log('game_status', gameDetail.game_status);
    if (gameDetail.game_status === 2) {
      $('.turn').text('Draw !');
      $('.detail').css({display: 'none'})
      gameId = null;
    } else if (gameDetail.game_status === 1) {
      $('.turn').text(`${gameDetail.winner} win!`);
      $('.detail').css({display: 'none'})
      gameId = null;
    }
  }
}
near_connect();

startGame()

function startGame() {
  for (let i = 0; i < cellElements.length; i++) {
    $(cellElements[i]).removeClass(X_CLASS)
    $(cellElements[i]).removeClass(CIRCLE_CLASS)
  }
}

cellElements.click(async function () {
  var index = cellElements.index(this);
  console.log('click', index);
  if (gameDetail.game[index] == 0) {
    try {
      loading(true)
      let response = await contract.play({'game_id': gameId, 'position': index});
      await updateGame();
      console.log('response', response);
      loading(false)
    } catch (e) {
      loading(false)
      console.log('call play', e.message);
      if (e.message.toString().includes('NOT_IS_YOUR_TURN') > 0) {
        alert('NOT_IS_YOUR_TURN');
      } else if (e.message.toString().includes('GAME_WAS_ENDED') > 0) {
        alert('GAME_WAS_ENDED');
      }
    }
  }
});

function loading(isShow) {
  console.log('loading', isShow)
  if (!isShow) {
    $('#page-loader').css({display: 'none'})
  } else {
    $('#page-loader').css({display: 'flex'})
  }
}

$('#newGame').click(function () {
  checkAccount();
})
async function checkAccount() {
  loading(true);
  let player2 = $('#input_player2').val();
  console.log('player2', player2)
  const account = await near.account(player2);
  console.log('account', account);
  try {
    const accdetail = await account.getAccountBalance();
    console.log('accdetail', accdetail);
    gameId = await contract.new_game({'second_player': player2});
    console.log('new game response: ', gameId);
    await updateGame();
  } catch (e) {
    alert(e.message);
  } finally {
    loading(false);
  }
}

setInterval(function(){
  if (gameId !== undefined && gameId !== null) {
    console.log('auto update after 5s');
    updateGame();
  }
}, 3000);

$('#connetWallet').on('click', function () {
  wallet.requestSignIn("inti_demo.testnet","Tic Tac Toe");
});
// $( "#connetWallet" ).click(function() {
//   alert( "Handler for .click() called." );
// });

// connect to NEAR
// const near = await connect(config);
//
// // create wallet connection
// const wallet = new WalletConnection(near);